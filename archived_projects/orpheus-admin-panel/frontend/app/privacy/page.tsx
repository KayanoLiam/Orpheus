// app/privacy/page.tsx

import type { NextPage } from 'next';
import Link from 'next/link';

// 定义一个可重用的组件来加粗关键字，使其更易于维护
const Strong = ({ children }: { children: React.ReactNode }) => {
  return <strong className="font-semibold text-white">{children}</strong>;
};

const PrivacyPolicyPage: NextPage = () => {
  return (
    // 主容器，提供垂直内边距和黑色背景（与导航栏一致）
    <div className="min-h-screen bg-black py-16 sm:py-24">
      {/* 
        内容容器
        - mx-auto: 水平居中
        - max-w-4xl: 设置最大宽度，防止文本过长，提升可读性
        - px-6 lg:px-8: 设置响应式的水平内边距
      */}
      <main className="mx-auto max-w-4xl px-6 lg:px-8">
        <div className="space-y-12">
          {/* 页面标题部分 */}
          <header>
            <h1 className="text-4xl font-bold tracking-tight text-white sm:text-5xl">
              プライバシーポリシー
            </h1>
            <p className="mt-4 text-gray-400">
              最終更新日: 2025年7月11日
            </p>
          </header>

          {/* 
            内容部分
            - space-y-8: 为直接子元素之间添加垂直间距
            - text-gray-300: 设置默认段落文字颜色
            - leading-relaxed: 增加行高，使长文本更易于阅读
          */}
          <article className="space-y-8 text-gray-300 leading-relaxed">
            <p>
              Orpheusプロジェクト（以下<Strong>「当方」</Strong>）は、お客様の個人情報の保護を重要視しています。このプライバシーポリシー（以下<Strong>「本ポリシー」</Strong>）は、当方が提供するサービス（以下<Strong>「本サービス」</Strong>）を通じて収集する個人情報の取り扱いについて説明します。
            </p>

            {/* 段落之间的分割线，可选，但可以增加视觉分隔 */}
            <hr className="border-white/10" />

            <section className="space-y-6">
              <h2 className="text-2xl font-bold text-white">
                収集する情報
              </h2>
              <p>
                当方は、本サービスの提供および改善のために、以下の情報を収集することがあります：
              </p>
              <ul className="list-disc list-inside space-y-2 ml-4">
                <li>アカウント情報：ユーザー名、メールアドレス、パスワード（ハッシュ化された形式）</li>
                <li>使用状況データ：ログイン時間、使用機能、アクセス履歴</li>
                <li>技術情報：IPアドレス、ブラウザ情報、デバイス情報</li>
                <li>通信内容：サービス提供に必要な最小限の通信データ</li>
              </ul>
            </section>

            <section className="space-y-6">
              <h2 className="text-2xl font-bold text-white">
                情報の利用目的
              </h2>
              <p>
                収集した情報は、以下の目的のために利用されます：
              </p>
              <ul className="list-disc list-inside space-y-2 ml-4">
                <li>本サービスの提供、運営、維持</li>
                <li>お客様へのサポート提供</li>
                <li>サービスの改善和新機能の開発</li>
                <li>セキュリティ維持と不正利用防止</li>
                <li>法的義務の遵守</li>
              </ul>
            </section>

            <section className="space-y-6">
              <h2 className="text-2xl font-bold text-white">
                情報の共有と開示
              </h2>
              <p>
                当方は、お客様の個人情報を第三者と共有することはありません。ただし、以下の場合は例外となります：
              </p>
              <ul className="list-disc list-inside space-y-2 ml-4">
                <li>お客様の明示的な同意がある場合</li>
                <li>法令に基づき開示が要求される場合</li>
                <li>当方の権利、財産、安全を保護するために必要な場合</li>
                <li>信頼できる第三者に業務委託する場合（適切な秘密保持契約を締結）</li>
              </ul>
            </section>

            <section className="space-y-6">
              <h2 className="text-2xl font-bold text-white">
                データの保管
              </h2>
              <p>
                収集した個人情報は、本サービスの提供に必要な期間のみ保管します。お客様がアカウントを削除された場合、関連する個人情報は合理的な期間内に削除または匿名化されます。
              </p>
            </section>

            <section className="space-y-6">
              <h2 className="text-2xl font-bold text-white">
                お客様の権利
              </h2>
              <p>
                お客様は、自身の個人情報に関して以下の権利を有します：
              </p>
              <ul className="list-disc list-inside space-y-2 ml-4">
                <li>個人情報の開示請求権</li>
                <li>個人情報の訂正・削除請求権</li>
                <li>個人情報の利用停止請求権</li>
                <li>個人情報の第三者提供停止請求権</li>
              </ul>
            </section>

            <section className="space-y-6">
              <h2 className="text-2xl font-bold text-white">
                セキュリティ対策
              </h2>
              <p>
                当方は、お客様の個人情報を保護するために、適切な技術的・組織的セキュリティ対策を実施しています。これには、データ暗号化、アクセス制御、定期的なセキュリティ監査などが含まれますが、これらに限定されません。
              </p>
            </section>

            <section className="space-y-6">
              <h2 className="text-2xl font-bold text-white">
                お問い合わせ
              </h2>
              <p>
                本ポリシーに関するご質問や、個人情報に関するお問い合わせは、以下の連絡先までお願いいたします：
              </p>
              <div className="bg-gray-800 p-4 rounded-lg">
                <p className="text-white">メールアドレス: privacy@orpheus.dev</p>
                <p className="text-white mt-2">担当者: 茅野（プロジェクト開発者）</p>
              </div>
            </section>

            <section className="space-y-6">
              <h2 className="text-2xl font-bold text-white">
                ポリシーの変更
              </h2>
              <p>
                当方は、必要に応じて本ポリシーを変更することがあります。重要な変更がある場合、本サービス内での通知またはその他の適切な方法でお客様にお知らせします。
              </p>
            </section>
          </article>
        </div>
      </main>
    </div>
  );
};

export default PrivacyPolicyPage;
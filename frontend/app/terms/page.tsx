// app/terms/page.tsx

import type { NextPage } from 'next';
import Link from 'next/link';

// 定义一个可重用的组件来加粗关键字，使其更易于维护
const Strong = ({ children }: { children: React.ReactNode }) => {
  return <strong className="font-semibold text-white">{children}</strong>;
};

const TermsOfServicePage: NextPage = () => {
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
              利用規約
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
              この利用規約（以下<Strong>「本規約」</Strong>）は、あなた（<Strong>「利用者」</Strong>、<Strong>「お客様」</Strong>）とOrpheusプロジェクト（<Strong>「当方」</Strong>、<Strong>「私たち」</Strong>）との間の拘束力のある契約です。本規約は、Orpheusサービスへのアクセスと使用を規定します。
            </p>

            {/* 段落之间的分割线，可选，但可以增加视觉分隔 */}
            <hr className="border-white/10" />

            <section className="space-y-6">
              <h2 className="text-2xl font-bold text-white">
                規約の同意
              </h2>
              <p>
                本規約は、サインアップ時に規約に同意するか、サービスにアクセスまたは使用することで効力が生じます（<Strong>「発効日」</Strong>）。サインアップ時に規約に同意するか、サービスにアクセスまたは使用することにより、あなたは（A）本規約を読んで理解したことを認め；（B）本規約に締結する権限、権力、および権限があることを表明し保証し、組織のために本規約に締結する場合、その組織を拘束する法的権限があることを保証し；（C）本規約に同意し、その条項に法的に拘束されることに同意します。
              </p>
              <p>
                各条項を理解していることを確認するため、この規約を注意深くお読みください。本規約には、あなたと私たちの間の紛争を解決するために、個別の最終的拘束力のある仲裁の排他的使用を要求する必須の個別仲裁規定（<Strong>「仲裁合意」</Strong>）と、集団訴訟/陪審裁判放棄規定（<Strong>「集団訴訟/陪審裁判放棄」</Strong>）が含まれています。適用法によって許可される最大限の範囲で、あなたは法廷で救済を求める権利と請求に対する陪審裁判を受ける権利、および集団訴訟、集団的、私人検事総長、または代表訴訟または手続きにおいて原告または集団メンバーとして参加する権利を明示的に放棄します。
              </p>
            </section>
          </article>
        </div>
      </main>
    </div>
  );
};

export default TermsOfServicePage;